import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { getSessionUser } from '$lib/auth/session';
import { sanitizeAuthPath } from '$lib/utils/auth-redirect';

export const load: PageServerLoad = async ({ cookies, params, url }) => {
  const sessionUser = getSessionUser(cookies.get('FERRISKEY_IDENTITY'));
  const clientId = url.searchParams.get('client_id');
  const redirectUri = url.searchParams.get('redirect_uri');
  const next = url.searchParams.get('next');
  const fallbackPath = `/realms/${params.realm}/overview`;
  const sanitizedNext = sanitizeAuthPath(next, fallbackPath);

  if (sessionUser && !clientId && !redirectUri) {
    throw redirect(303, sanitizedNext);
  }

  if (!clientId || !redirectUri) {
    // Redirect through our own /api proxy so the FERRISKEY_SESSION cookie
    // stays on the dashboard domain instead of the backend domain.
    const authUrl = new URL(
      `${url.origin}/api/realms/${params.realm}/protocol/openid-connect/auth`
    );

    authUrl.searchParams.set('response_type', 'code');
    authUrl.searchParams.set('client_id', 'security-admin-console');
    authUrl.searchParams.set(
      'redirect_uri',
      `${url.origin}${sanitizedNext}`
    );
    authUrl.searchParams.set('scope', 'openid profile email');

    throw redirect(303, authUrl.toString());
  }

  return {
    clientId,
    redirectUri,
    next
  };
};
