import { env } from '$env/dynamic/public';

export function resolveApiBase(url: URL) {
  const apiUrl = env.PUBLIC_API_URL?.trim() ?? '';

  if (apiUrl.length > 0) {
    return apiUrl.replace(/\/+$/, '');
  }

  return `${url.origin.replace(/\/+$/, '')}/api`;
}
