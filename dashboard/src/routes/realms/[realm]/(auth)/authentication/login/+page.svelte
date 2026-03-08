<script lang="ts">
  import { page } from '$app/state';
  import { ArrowRight, ShieldCheck, Sparkles } from 'lucide-svelte';
  import BrandLogo from '$components/BrandLogo.svelte';
  import ThemeToggle from '$components/ThemeToggle.svelte';
  import { ripple } from '$utils/ripple';

  const realmName = String(page.params.realm ?? 'master');
  const benefits = [
    'Minimal Design visual system across auth and admin pages',
    'Clearer MFA and required action entry points',
    'Barrzen branding with SSR-ready foundations'
  ];
</script>

<div class="login-page">
  <div class="login-page__topbar">
    <BrandLogo />
    <ThemeToggle />
  </div>

  <section class="login-page__card glass-panel">
    <div class="login-page__intro">
      <div class="login-page__eyebrow">
        <Sparkles size={16} />
        Barrzen Minimal auth
      </div>
      <h1>Sign in to {realmName}.</h1>
      <p>
        The new login surface carries the same Barrzen Minimal language as the
        dashboard: clean hierarchy, soft contrast, and simpler action focus.
      </p>

      <div class="login-page__feature-card">
        <div>
          <small>Realm posture</small>
          <strong>Secure and ready</strong>
        </div>
        <div class="login-page__pulse">
          <span class="status-dot"></span>
          SSO health 99.94%
        </div>
      </div>

      <div class="login-page__benefits">
        {#each benefits as benefit (benefit)}
          <div>
            <ShieldCheck size={16} color="var(--primary)" />
            <span>{benefit}</span>
          </div>
        {/each}
      </div>
    </div>

    <form class="login-page__form">
      <div class="login-page__form-head">
        <strong>Welcome back</strong>
        <span>Use your realm credentials to continue.</span>
      </div>

      <label>
        <span>Email or username</span>
        <input type="text" placeholder="admin or admin@example.com" />
      </label>
      <label>
        <span>Password</span>
        <input type="password" placeholder="Enter your password" />
      </label>
      <label>
        <span>Realm</span>
        <input type="text" value={realmName} readonly />
      </label>

      <button type="button" class="login-page__submit" use:ripple>
        Continue
        <ArrowRight size={16} />
      </button>

      <a href={`/realms/${realmName}/overview`} class="login-page__link"
        >Skip to dashboard shell</a
      >
    </form>
  </section>
</div>

<style>
  .login-page {
    min-height: 100vh;
    padding: 16px;
    display: grid;
    gap: 20px;
  }

  .login-page__topbar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 16px;
  }

  .login-page__card {
    max-width: 1120px;
    margin: 0 auto;
    padding: clamp(20px, 4vw, 32px);
    display: grid;
    grid-template-columns: minmax(0, 1.15fr) minmax(320px, 420px);
    gap: 28px;
  }

  .login-page__intro,
  .login-page__benefits,
  .login-page__form {
    display: grid;
    gap: 18px;
  }

  .login-page__eyebrow {
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
    font: 700 clamp(2.2rem, 5vw, 4.2rem)/0.95 var(--font-display);
    letter-spacing: -0.06em;
  }

  p,
  .login-page__link,
  label span,
  .login-page__form-head span,
  .login-page__pulse,
  .login-page__feature-card small {
    color: var(--text-soft);
  }

  p {
    line-height: 1.7;
  }

  .login-page__feature-card {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 16px;
    padding: 18px;
    border-radius: 20px;
    background: linear-gradient(
      180deg,
      color-mix(in srgb, var(--primary) 10%, var(--surface)) 0%,
      var(--surface) 100%
    );
    border: 1px solid var(--border);
  }

  .login-page__feature-card strong,
  .login-page__form-head strong {
    display: block;
    font: 700 1.2rem/1.1 var(--font-display);
  }

  .login-page__pulse {
    display: inline-flex;
    align-items: center;
    gap: 10px;
    padding: 10px 12px;
    border-radius: 999px;
    background: var(--surface);
    border: 1px solid var(--border);
  }

  .login-page__benefits div {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 16px 18px;
    border-radius: 18px;
    background: var(--bg-inset);
    border: 1px solid var(--border);
  }

  .login-page__form {
    padding: 24px;
    border-radius: 24px;
    background: var(--surface);
    border: 1px solid var(--border);
    box-shadow: var(--shadow-md);
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

  input:focus {
    border-color: color-mix(in srgb, var(--primary) 55%, transparent);
    box-shadow: 0 0 0 4px color-mix(in srgb, var(--primary) 16%, transparent);
  }

  .login-page__submit {
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

  .login-page__link {
    justify-self: center;
    font-weight: 700;
  }

  @media (max-width: 900px) {
    .login-page__topbar {
      flex-direction: column;
      align-items: flex-start;
    }

    .login-page__card {
      grid-template-columns: 1fr;
    }

    .login-page__feature-card {
      flex-direction: column;
      align-items: flex-start;
    }
  }
</style>
