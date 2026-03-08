<script lang="ts">
  import { goto } from '$app/navigation';
  import { page } from '$app/state';
  import { ApiError, apiRequest } from '$lib/api/client';
  import BrandLogo from '$components/BrandLogo.svelte';
  import ThemeToggle from '$components/ThemeToggle.svelte';
  import { ripple } from '$utils/ripple';
  import { ArrowRight, ShieldCheck } from 'lucide-svelte';

  const realmName = $derived(String(page.params.realm ?? 'master'));
  const token = $derived(page.url.searchParams.get('token') ?? '');
  let code = $state('');
  let errorMessage = $state('');
  let isSubmitting = $state(false);

  function decodeEmail(value: string) {
    try {
      const payload = value.split('.')[1]?.replace(/-/g, '+').replace(/_/g, '/');

      if (!payload) {
        return null;
      }

      const parsed = JSON.parse(atob(payload.padEnd(Math.ceil(payload.length / 4) * 4, '='))) as {
        email?: string;
      };

      return parsed.email ?? null;
    } catch {
      return null;
    }
  }

  const email = $derived(decodeEmail(token));

  async function submitOtp(event: SubmitEvent) {
    event.preventDefault();

    if (!token) {
      errorMessage = 'The OTP session token is missing. Start the sign-in flow again.';
      return;
    }

    errorMessage = '';
    isSubmitting = true;

    try {
      const response = await apiRequest<{ url: string }>({
        url: page.url,
        fetcher: fetch,
        path: `/realms/${realmName}/login-actions/challenge-otp`,
        init: {
          method: 'POST',
          headers: {
            Authorization: `Bearer ${token}`
          },
          body: JSON.stringify({ code })
        }
      });

      window.location.href = response.url;
    } catch (error) {
      errorMessage =
        error instanceof ApiError
          ? error.message
          : 'The OTP code could not be verified. Please try again.';
    } finally {
      isSubmitting = false;
    }
  }
</script>

<div class="otp-page">
  <div class="otp-page__topbar">
    <BrandLogo />
    <ThemeToggle />
  </div>

  <section class="otp-page__card glass-panel">
    <div class="otp-page__intro">
      <div class="otp-page__eyebrow">
        <ShieldCheck size={16} />
        One more security step
      </div>
      <h1>Enter your OTP code.</h1>
      <p>
        Use the current 6-digit code from your authenticator app to finish signing in.
        {#if email}<span> This challenge is tied to {email}.</span>{/if}
      </p>
    </div>

    <form class="otp-page__form" onsubmit={submitOtp}>
      <label>
        <span>Authenticator code</span>
        <input
          type="text"
          inputmode="numeric"
          maxlength="6"
          minlength="6"
          placeholder="000000"
          bind:value={code}
          required
        />
      </label>

      {#if errorMessage}
        <p class="otp-page__error">{errorMessage}</p>
      {/if}

      <button type="submit" class="otp-page__submit" use:ripple disabled={isSubmitting}>
        {isSubmitting ? 'Verifying...' : 'Finish sign-in'}
        <ArrowRight size={16} />
      </button>

      <button
        type="button"
        class="otp-page__cancel"
        onclick={() => goto(`/realms/${realmName}/authentication/login`)}
      >
        Cancel
      </button>
    </form>
  </section>
</div>

<style>
  .otp-page {
    min-height: 100vh;
    padding: 16px;
    display: grid;
    gap: 20px;
  }

  .otp-page__topbar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 16px;
  }

  .otp-page__card {
    width: min(100%, 920px);
    margin: 0 auto;
    padding: clamp(20px, 4vw, 32px);
    display: grid;
    grid-template-columns: minmax(0, 1fr) minmax(280px, 380px);
    gap: 24px;
  }

  .otp-page__intro,
  .otp-page__form {
    display: grid;
    gap: 18px;
  }

  .otp-page__eyebrow {
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
    font: 700 clamp(2rem, 4vw, 3.4rem)/0.96 var(--font-display);
    letter-spacing: -0.06em;
  }

  p,
  span {
    margin: 0;
    color: var(--text-soft);
    line-height: 1.7;
  }

  .otp-page__form {
    padding: 24px;
    border-radius: 24px;
    background: var(--surface);
    border: 1px solid var(--border);
    box-shadow: var(--shadow-md);
    align-content: start;
  }

  label {
    display: grid;
    gap: 8px;
  }

  input,
  .otp-page__cancel {
    width: 100%;
    padding: 14px 16px;
    border-radius: 16px;
    border: 1px solid var(--border);
    background: var(--surface-strong);
    color: var(--text);
    outline: 0;
    text-align: center;
    letter-spacing: 0.24em;
    font-size: 1.2rem;
    font-weight: 700;
  }

  .otp-page__submit {
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

  .otp-page__submit:disabled {
    opacity: 0.7;
    cursor: wait;
  }

  .otp-page__cancel {
    letter-spacing: normal;
    font-size: 0.95rem;
    font-weight: 700;
    cursor: pointer;
  }

  .otp-page__error {
    padding: 12px 14px;
    border-radius: 14px;
    background: color-mix(in srgb, #d14343 12%, var(--surface));
    border: 1px solid color-mix(in srgb, #d14343 28%, transparent);
    color: #b42318;
  }

  @media (max-width: 900px) {
    .otp-page__topbar {
      flex-direction: column;
      align-items: flex-start;
    }

    .otp-page__card {
      grid-template-columns: 1fr;
    }
  }
</style>
