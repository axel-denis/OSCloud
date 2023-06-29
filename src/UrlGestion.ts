export interface UrlInfo {
  app: string;
  parameters: string[] | null
}

export interface UrlsHandler {
  actualUrl: UrlInfo;
  nextUrl: UrlInfo;
}

export type AnimationStates = "intro" | "inter" | "outro";

export function urlToInfo(url: string): UrlInfo {
  if (url == "") {
    return { app: "", parameters: null }
  }
  if (url === "/") {
    return { app: "Home", parameters: null }
  }
  while (url[0] === '/') url = url.substring(1);
  const splitted = url.split("/");
  return {app: splitted[0], parameters: splitted.slice(1)};
}

export function transitionToUrl(handler: UrlsHandler, setHandler: React.Dispatch<React.SetStateAction<UrlsHandler>>, newUrl: string, duration: number) {
  setHandler({...handler, nextUrl: urlToInfo(newUrl)});
  setTimeout(() => {
    setHandler({actualUrl: urlToInfo(newUrl), nextUrl: urlToInfo("")});
  }, duration);
}

export function discreetlyChangeUrlPath(path: string) {
  window.history.replaceState(null, "", path)
}

export function getAnimationState(urlsHandler: UrlsHandler, appName: string): AnimationStates {
  if (urlsHandler.actualUrl.app === appName) {
    if (urlsHandler.nextUrl.app == null || urlsHandler.nextUrl.app == "") {
      return "inter";
    } else {
      return "outro";
    }
  } else if (urlsHandler.nextUrl.app === appName) {
    return "intro";
  }
  return "inter";
}