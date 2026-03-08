import { z } from 'zod';
import { resolveApiBase } from '$lib/api/config';

const errorSchema = z.object({
  message: z.string().optional(),
  error: z.string().optional()
});

export class ApiError extends Error {
  status: number;

  constructor(message: string, status: number) {
    super(message);
    this.name = 'ApiError';
    this.status = status;
  }
}

export async function apiRequest<T>(input: {
  url: URL;
  fetcher: typeof fetch;
  path: string;
  init?: RequestInit;
}): Promise<T> {
  const response = await input.fetcher(
    `${resolveApiBase(input.url)}${input.path}`,
    {
      credentials: 'include',
      headers: {
        'Content-Type': 'application/json',
        ...(input.init?.headers ?? {})
      },
      ...input.init
    }
  );

  if (!response.ok) {
    const payload = errorSchema.safeParse(
      await response.json().catch(() => null)
    );
    throw new ApiError(
      payload.data?.message ?? payload.data?.error ?? 'Request failed',
      response.status
    );
  }

  if (response.status === 204) {
    return undefined as T;
  }

  return (await response.json()) as T;
}
