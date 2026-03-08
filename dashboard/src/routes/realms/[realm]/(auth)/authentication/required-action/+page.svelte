<script lang="ts">
  import { goto } from '$app/navigation';
  import { page } from '$app/state';
  import { ApiError, apiRequest } from '$lib/api/client';
  import BrandLogo from '$components/BrandLogo.svelte';
  import ThemeToggle from '$components/ThemeToggle.svelte';
  import { ripple } from '$utils/ripple';
  import { ArrowRight, KeyRound, ShieldCheck } from 'lucide-svelte';

  type AuthenticateResponse = {
    message?: string | null;
    required_actions?: string[] | null;
    status: 'Success' | 'RequiresActions' | 'RequiresOtpChallenge' | 'Failed';
    token?: string | null;
    url?: string | null;
  };

  type SetupOtpResponse = {
    issuer: string;
    otpauth_url: string;
    secret: string;
  };

  const realmName = $derived(String(page.params.realm ?? 'master'));
  const execution = $derived(
    (page.url.searchParams.get('execution') ?? '').toLowerCase()
  );
  const token = $derived(page.url.searchParams.get('client_data') ?? '');
  let password = $state('');
  let confirmPassword = $state('');
  let deviceName = $state('');
  let otpCode = $state('');
  let errorMessage = $state('');
  let otpSetup = $state<SetupOtpResponse | null>(null);
  let otpLoading = $state(false);
  let isSubmitting = $state(false);

  async function continueAuthentication() {
    const response = await apiRequest<AuthenticateResponse>({
      url: page.url,
      fetcher: fetch,
      path: `/realms/${realmName}/login-actions/authenticate?client_id=security-admin-console`,
      init: {
        method: 'POST',
        headers: {
          Authorization: `Bearer ${token}`
        },
        body: JSON.stringify({})
      }
    });

    if (response.url) {
      window.location.href = response.url;
      return;
    }

    if (response.status === 'RequiresOtpChallenge' && response.token) {
      await goto(`/realms/${realmName}/authentication/otp?token=${encodeURIComponent(response.token)}`);
      return;
    }

    if (response.status === 'RequiresActions' && response.token && response.required_actions?.length) {
      const nextAction = response.required_actions[0];
      await goto(
        `/realms/${realmName}/authentication/required-action?execution=${encodeURIComponent(nextAction.toUpperCase())}&client_data=${encodeURIComponent(response.token)}`
      );
      return;
    }

    await goto(`/realms/${realmName}/overview`, { invalidateAll: true });
  }

  $effect(() => {
    if (execution !== 'configure_otp' || !token || otpSetup || otpLoading) {
      return;
    }

    otpLoading = true;
    errorMessage = '';

    void apiRequest<SetupOtpResponse>({
      url: page.url,
      fetcher: fetch,
      path: `/realms/${realmName}/login-actions/setup-otp`,
      init: {
        headers: {
          Authorization: `Bearer ${token}`
        }
      }
    })
      .then((response) => {
        otpSetup = response;
      })
      .catch((error) => {
        errorMessage =
          error instanceof ApiError
            ? error.message
            : 'The OTP setup payload could not be loaded.';
      })
      .finally(() => {
        otpLoading = false;
      });
  });

  async function submitUpdatePassword(event: SubmitEvent) {
    event.preventDefault();

    if (!token) {
      errorMessage = 'The required-action token is missing. Start the sign-in flow again.';
      return;
    }

    if (password.length < 8) {
      errorMessage = 'Choose a password with at least 8 characters.';
      return;
    }

    if (password !== confirmPassword) {
      errorMessage = 'The password confirmation does not match.';
      return;
    }

    errorMessage = '';
    isSubmitting = true;

    try {
      await apiRequest<{ message: string }>({
        url: page.url,
        fetcher: fetch,
        path: `/realms/${realmName}/login-actions/update-password`,
        init: {
          method: 'POST',
          headers: {
            Authorization: `Bearer ${token}`
          },
          body: JSON.stringify({ value: password })
        }
      });

      await continueAuthentication();
    } catch (error) {
      errorMessage =
        error instanceof ApiError
          ? error.message
          : 'The password update could not be completed.';
    } finally {
      isSubmitting = false;
    }
  }

  async function submitConfigureOtp(event: SubmitEvent) {
    event.preventDefault();

    if (!token || !otpSetup) {
      errorMessage = 'OTP setup information is missing. Start the flow again.';
      return;
    }

    if (otpCode.length !== 6) {
      errorMessage = 'Enter the 6-digit code from your authenticator app.';
      return;
    }

    errorMessage = '';
    isSubmitting = true;

    try {
      await apiRequest<{ message: string }>({
        url: page.url,
        fetcher: fetch,
        path: `/realms/${realmName}/login-actions/verify-otp`,
        init: {
          method: 'POST',
          headers: {
            Authorization: `Bearer ${token}`
          },
          body: JSON.stringify({
            code: otpCode,
            label: deviceName || 'Barrzen Minimal device',
            secret: otpSetup.secret
          })
        }
      });

      await continueAuthentication();
    } catch (error) {
      errorMessage =
        error instanceof ApiError
          ? error.message
          : 'OTP verification failed. Please try again.';
    } finally {
      isSubmitting = false;
    }
  }
</script>

<div class="required-page">
  <div class="required-page__topbar">
    <BrandLogo />
    <ThemeToggle />
  </div>

  <section class="required-page__card glass-panel">
    <div class="required-page__intro">
      <div class="required-page__eyebrow">
        <ShieldCheck size={16} />
        Required action
      </div>

      {#if execution === 'configure_otp'}
        <h1>Enable OTP before you continue.</h1>
        <p>
          Add a TOTP device for this account, verify the first code, and the sign-in flow will resume automatically.
        </p>
      {:else if execution === 'update_password'}
        <h1>Update your password.</h1>
        <p>
          This account needs a fresh password before the session can continue into the new dashboard.
        </p>
      {:else}
        <h1>This action has not been mapped yet.</h1>
        <p>
          The current required action is not migrated in the new UI yet. Return to login and restart the flow if needed.
        </p>
      {/if}
    </div>

    {#if execution === 'configure_otp'}
      <form class="required-page__form" onsubmit={submitConfigureOtp}>
        <div class="required-page__panel">
          <strong>Authenticator setup</strong>
          <small>Use the secret below in any TOTP-compatible authenticator app.</small>
        </div>

        <div class="required-page__secret">
          <span>Shared secret</span>
          <code>{otpSetup?.secret ?? 'Loading...'}</code>
          {#if otpSetup?.otpauth_url}
            <small>{otpSetup.otpauth_url}</small>
          {/if}
        </div>

        <label>
          <span>Device label</span>
          <input type="text" bind:value={deviceName} placeholder="Work phone" />
        </label>

        <label>
          <span>Verification code</span>
          <input
            type="text"
            inputmode="numeric"
            maxlength="6"
            minlength="6"
            placeholder="000000"
            bind:value={otpCode}
            required
          />
        </label>

        {#if errorMessage}
          <p class="required-page__error">{errorMessage}</p>
        {/if}

        <button type="submit" class="required-page__submit" use:ripple disabled={isSubmitting || otpLoading}>
          {isSubmitting ? 'Verifying...' : 'Enable OTP'}
          <ArrowRight size={16} />
        </button>
      </form>
    {:else if execution === 'update_password'}
      <form class="required-page__form" onsubmit={submitUpdatePassword}>
        <div class="required-page__panel">
          <strong>Fresh credentials</strong>
          <small>Pick a new password and confirm it to continue.</small>
        </div>

        <label>
          <span>New password</span>
          <input
            type="password"
            bind:value={password}
            autocomplete="new-password"
            placeholder="Choose a strong password"
            required
          />
        </label>

        <label>
          <span>Confirm password</span>
          <input
            type="password"
            bind:value={confirmPassword}
            autocomplete="new-password"
            placeholder="Repeat the password"
            required
          />
        </label>

        {#if errorMessage}
          <p class="required-page__error">{errorMessage}</p>
        {/if}

        <button type="submit" class="required-page__submit" use:ripple disabled={isSubmitting}>
          {isSubmitting ? 'Saving...' : 'Update password'}
          <KeyRound size={16} />
        </button>
      </form>
    {:else}
      <div class="required-page__form required-page__form--empty">
        <button
          type="button"
          class="required-page__submit"
          use:ripple
          onclick={() => goto(`/realms/${realmName}/authentication/login`)}
        >
          Return to login
        </button>
      </div>
    {/if}
  </section>
</div>

<style>
  .required-page {
    min-height: 100vh;
    padding: 16px;
    display: grid;
    gap: 20px;
  }

  .required-page__topbar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 16px;
  }

  .required-page__card {
    max-width: 1120px;
    margin: 0 auto;
    padding: clamp(20px, 4vw, 32px);
    display: grid;
    grid-template-columns: minmax(0, 1.05fr) minmax(320px, 420px);
    gap: 28px;
  }

  .required-page__intro,
  .required-page__form {
    display: grid;
    gap: 18px;
  }

  .required-page__eyebrow {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    padding: 9px 14px;
    border-radius: 999px;
    background: var(--primary-soft);
    color: var(--primary);
    font-size: 0.86rem;
    font-weight: 700;
    justify-self: start;
  }

  h1 {
    margin: 0;
    font: 700 clamp(2rem, 4vw, 3.8rem)/0.95 var(--font-display);
    letter-spacing: -0.06em;
  }

  p,
  small,
  span {
    margin: 0;
    color: var(--text-soft);
    line-height: 1.7;
  }

  .required-page__form {
    padding: 24px;
    border-radius: 24px;
    background: var(--surface);
    border: 1px solid var(--border);
    box-shadow: var(--shadow-md);
    align-content: start;
  }

  .required-page__form--empty {
    justify-content: center;
  }

  .required-page__panel,
  .required-page__secret {
    display: grid;
    gap: 6px;
    padding: 16px;
    border-radius: 18px;
    background: var(--bg-inset);
    border: 1px solid var(--border);
  }

  .required-page__secret code {
    padding: 10px 12px;
    border-radius: 14px;
    background: var(--surface);
    color: var(--text);
    overflow-wrap: anywhere;
  }

  label {
    display: grid;
    gap: 8px;
  }

  input {
    width: 100%;
    padding: 14px 16px;
    border-radius: 16px;
    border: 1px solid var(--border);
    background: var(--surface-strong);
    color: var(--text);
    outline: 0;
  }

  .required-page__submit {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 10px;
    padding: 15px 18px;
    border-radius: 16px;
    border: 0;
    background: var(--primary);
    color: white;
    font-weight: 700;
    cursor: pointer;
  }

  .required-page__submit:disabled {
    opacity: 0.7;
    cursor: wait;
  }

  .required-page__error {
    padding: 12px 14px;
    border-radius: 14px;
    background: color-mix(in srgb, #d14343 12%, var(--surface));
    border: 1px solid color-mix(in srgb, #d14343 28%, transparent);
    color: #b42318;
  }

  @media (max-width: 900px) {
    .required-page__topbar {
      flex-direction: column;
      align-items: flex-start;
    }

    .required-page__card {
      grid-template-columns: 1fr;
    }
  }
</style>
