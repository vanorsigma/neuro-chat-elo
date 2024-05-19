// If the user is going to experience a degraded experience due to
// certain features not being supported, tell them.
// TODO: use somewhere, and ensure it stays updated

export function checkAPIs() {
  const apis = [
    'IntersectionObserver',
    'fetch',
    'Promise'
  ];

  const unsupportedAPIs = apis.filter((api: string) => {
    return typeof window[api] === 'undefined';
  });

  return unsupportedAPIs;
}
