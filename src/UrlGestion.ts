export interface UrlInfo {
  app: string;
  parameters: string[] | null
}

export interface UrlsHandler {
  actualUrl: UrlInfo;
  nextUrl: UrlInfo;
}

export function urlToInfo(url: string): UrlInfo {
  while (url[0] === '/') url = url.substring(1);
  const splitted = url.split("/");
  return {app: splitted[0], parameters: splitted.slice(1)};
}