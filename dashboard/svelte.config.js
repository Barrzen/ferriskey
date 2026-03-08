import adapter from '@sveltejs/adapter-node';

/** @type {import('@sveltejs/kit').Config} */
const config = {
  kit: {
    adapter: adapter({
      precompress: true
    }),
    alias: {
      $components: 'src/lib/components',
      $config: 'src/lib/config',
      $utils: 'src/lib/utils'
    }
  }
};

export default config;
