export interface UrlInfo {
  app: string;
  parameters: string[] | null
}

export interface UrlsHandler {
  actualUrl: UrlInfo;
  nextUrl: UrlInfo;
  afterUrl: UrlInfo;
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
  return { app: splitted[0], parameters: splitted.slice(1) };
}

export function transitionToUrl(handler: UrlsHandler, setHandler: React.Dispatch<React.SetStateAction<UrlsHandler>>, newUrl: string, duration: number) {
  if (handler.nextUrl.app === "") {
    setHandler({ ...handler, nextUrl: urlToInfo(newUrl) });
    setTimeout(() => {
      console.log("1handler goind from to", handler, {...handler, actualUrl: urlToInfo(newUrl), nextUrl: urlToInfo("") })
      setHandler({...handler, actualUrl: urlToInfo(newUrl), nextUrl: urlToInfo("") });
    }, duration);
  } else {
    setHandler({ ...handler, afterUrl: urlToInfo(newUrl) });
    setTimeout(() => {
      console.log("1handler goind from to", handler, {...handler, actualUrl: urlToInfo(newUrl), nextUrl: urlToInfo("") })
      setHandler({actualUrl: urlToInfo(newUrl), nextUrl:urlToInfo(""), afterUrl: urlToInfo("") });
    }, duration);
  }
}

export function discreetlyChangeUrlPath(path: string) {
  window.history.replaceState(null, "", path)
}

export function getAnimationState(urlsHandler: UrlsHandler, appName: string): AnimationStates {
  console.log("asked for", urlsHandler)
  if (urlsHandler.nextUrl.app === appName || urlsHandler.afterUrl.app === appName) {
    console.log("returned", "intro" )
    return "intro";
  } else if (urlsHandler.actualUrl.app === appName && urlsHandler.nextUrl.app === "" && urlsHandler.afterUrl.app === "") {
    console.log("returned", "inter" )
    return "inter";
  }
  console.log("returned", "outro" )
  return "outro";
}