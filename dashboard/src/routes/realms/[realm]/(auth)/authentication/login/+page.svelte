<script lang="ts">
  import BrandMark from '$components/BrandMark.svelte';
  import ThemeToggle from '$components/ThemeToggle.svelte';
  import { ArrowRight, ShieldCheck } from 'lucide-svelte';
  import { page } from '$app/state';
  import { ripple } from '$utils/ripple';

  const benefits = [
    'Realm-aware authentication with modern SSR foundations',
    'Clearer MFA prompts and faster required action flows',
    'Minimal-inspired visuals tailored for FerrisKey operations'
  ];
</script>

<div class="login-page">
  <div class="login-page__topbar">
    <div class="login-page__brand">
      <BrandMark />
      <div>
        <strong>FerrisKey Dashboard</strong>
        <span>Preview build</span>
      </div>
    </div>
    <ThemeToggle />
  </div>

  <section class="login-page__card glass-panel">
    <div class="login-page__intro">
      <div class="login-page__eyebrow">
        <ShieldCheck size={16} />
        Realm security preview
      </div>
      <h1>Sign in to {page.params.realm}.</h1>
      <p>
        This preview page keeps the future login experience aligned with the new
        dashboard language while we wire the full server-managed auth flow.
      </p>

      <div class="login-page__benefits">
        {#each benefits as benefit}
          <div>
            <span class="status-dot"></span>
            <span>{benefit}</span>
          </div>
        {/each}
      </div>
    </div>

    <form class="login-page__form">
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
        <input type="text" value={page.params.realm} readonly />
      </label>

      <button type="button" class="login-page__submit" use:ripple>
        Continue to preview
        <ArrowRight size={16} />
      </button>

      <a href={`/realms/${page.params.realm}/overview`} class="login-page__link"
        >Skip to dashboard shell</a
      >
    </form>
  </section>
</div>

<style>
  .login-page {
    min-height: 100vh;
    padding: 24px;
    display: grid;
    align-content: start;
    gap: 28px;
  }

  .login-page__topbar,
  .login-page__brand,
  .login-page__benefits div {
    display: flex;
    align-items: center;
    gap: 14px;
  }

  .login-page__topbar {
    justify-content: space-between;
  }

  .login-page__brand span,
  .login-page__intro p,
  .login-page__link,
  label span {
    color: var(--text-soft);
  }

  .login-page__card {
    max-width: 1120px;
    margin: 0 auto;
    padding: clamp(24px, 4vw, 40px);
    display: grid;
    grid-template-columns: minmax(0, 1.1fr) minmax(320px, 440px);
    gap: 36px;
  }

  .login-page__eyebrow {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    padding: 8px 12px;
    border-radius: 999px;
    background: var(--primary-soft);
    color: var(--primary);
    font-size: 0.9rem;
    font-weight: 700;
  }

  h1 {
    margin: 18px 0 14px;
    font: 700 clamp(2.3rem, 5vw, 4.2rem)/0.94 var(--font-display);
    letter-spacing: -0.06em;
  }

  .login-page__intro p {
    max-width: 560px;
    line-height: 1.7;
  }

  .login-page__benefits {
    display: grid;
    gap: 16px;
    margin-top: 28px;
  }

  .login-page__benefits div {
    padding: 16px 18px;
    border-radius: 18px;
    background: var(--bg-inset);
    border: 1px solid var(--border);
  }

  .login-page__form {
    display: grid;
    gap: 18px;
    padding: 24px;
    border-radius: 26px;
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
    margin-top: 10px;
    padding: 15px 20px;
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
    .login-page,
    .login-page__card {
      padding: 16px;
    }

    .login-page__card {
      grid-template-columns: 1fr;
    }

    .login-page__topbar {
      flex-direction: column;
      align-items: flex-start;
    }
  }
</style>
