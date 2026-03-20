const resolvedDataUrls = new Map<string, string | null>();
const pendingDataUrls = new Map<string, Promise<string | null>>();

export async function getCachedDataUrl(
  cacheKey: string,
  loader: () => Promise<string | null>,
): Promise<string | null> {
  if (resolvedDataUrls.has(cacheKey)) {
    return resolvedDataUrls.get(cacheKey) ?? null;
  }

  const pending = pendingDataUrls.get(cacheKey);
  if (pending) {
    return pending;
  }

  const request = loader()
    .then((value) => {
      const normalized = value ?? null;
      resolvedDataUrls.set(cacheKey, normalized);
      pendingDataUrls.delete(cacheKey);
      return normalized;
    })
    .catch((error) => {
      pendingDataUrls.delete(cacheKey);
      throw error;
    });

  pendingDataUrls.set(cacheKey, request);
  return request;
}

export function evictCachedDataUrl(cacheKey: string): void {
  resolvedDataUrls.delete(cacheKey);
  pendingDataUrls.delete(cacheKey);
}

export function pruneCachedDataUrlsByPrefix(
  prefix: string,
  validCacheKeys: Iterable<string>,
): void {
  const valid = new Set(validCacheKeys);

  for (const cacheKey of resolvedDataUrls.keys()) {
    if (cacheKey.startsWith(prefix) && !valid.has(cacheKey)) {
      resolvedDataUrls.delete(cacheKey);
    }
  }

  for (const cacheKey of pendingDataUrls.keys()) {
    if (cacheKey.startsWith(prefix) && !valid.has(cacheKey)) {
      pendingDataUrls.delete(cacheKey);
    }
  }
}
