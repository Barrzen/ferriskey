type RippleOptions = {
  centered?: boolean;
};

export function ripple(node: HTMLElement, options: RippleOptions = {}) {
  const onPointerDown = (event: PointerEvent) => {
    const host = node;
    const rect = host.getBoundingClientRect();
    const rippleNode = document.createElement('span');
    const size = Math.max(rect.width, rect.height) * 1.1;
    const x = options.centered ? rect.width / 2 : event.clientX - rect.left;
    const y = options.centered ? rect.height / 2 : event.clientY - rect.top;

    rippleNode.className = 'ripple-wave';
    rippleNode.style.width = `${size}px`;
    rippleNode.style.height = `${size}px`;
    rippleNode.style.left = `${x - size / 2}px`;
    rippleNode.style.top = `${y - size / 2}px`;

    host.append(rippleNode);
    rippleNode.addEventListener('animationend', () => rippleNode.remove(), {
      once: true
    });
  };

  node.classList.add('ripple-host');
  node.addEventListener('pointerdown', onPointerDown);

  return {
    destroy() {
      node.removeEventListener('pointerdown', onPointerDown);
    }
  };
}
