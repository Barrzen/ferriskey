const INTERNAL_ORIGIN = 'http://dashboard.local';
const TRANSIENT_AUTH_PARAMS = ['code', 'state', 'session_state'];

function stripTransientAuthParams(searchParams: URLSearchParams) {
  for (const key of TRANSIENT_AUTH_PARAMS) {
    searchParams.delete(key);
  }
}

export function sanitizeAuthPath(
  rawPath: string | null | undefined,
  fallbackPath: string
) {
  if (!rawPath?.startsWith('/')) {
    return fallbackPath;
  }

  const parsed = new URL(rawPath, INTERNAL_ORIGIN);
  stripTransientAuthParams(parsed.searchParams);

  const sanitized = `${parsed.pathname}${parsed.search}${parsed.hash}`;
  return sanitized || fallbackPath;
}

export function sanitizeCurrentAppPath(url: URL) {
  const parsed = new URL(url.toString());
  stripTransientAuthParams(parsed.searchParams);
  return `${parsed.pathname}${parsed.search}${parsed.hash}`;
}

export function hasTransientAuthParams(url: URL) {
  return TRANSIENT_AUTH_PARAMS.some((key) => url.searchParams.has(key));
}
