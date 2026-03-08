<div class="brand-mark" aria-hidden="true">
  <div class="brand-mark__core"></div>
  <div class="brand-mark__accent"></div>
</div>

<style>
  .brand-mark {
    position: relative;
    width: 44px;
    height: 44px;
    border-radius: 16px;
    background: linear-gradient(
      145deg,
      var(--primary),
      color-mix(in srgb, var(--success) 42%, var(--primary))
    );
    box-shadow:
      inset 0 1px 0 rgba(255, 255, 255, 0.35),
      0 16px 32px color-mix(in srgb, var(--primary) 24%, transparent);
  }

  .brand-mark__core,
  .brand-mark__accent {
    position: absolute;
    border-radius: 999px;
    background: rgba(255, 255, 255, 0.9);
  }

  .brand-mark__core {
    inset: 11px;
    clip-path: polygon(0 0, 100% 0, 70% 52%, 100% 100%, 0 100%, 30% 50%);
  }

  .brand-mark__accent {
    width: 12px;
    height: 12px;
    right: 7px;
    top: 7px;
    opacity: 0.8;
  }
</style>
