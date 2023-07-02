export interface UrlInfo {
  app: string;
  parameters: string[] | null
}

export type AnimationStates = "intro" | "inter" | "outro" | "off";

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

export function transitionToUrl(handler: UrlInfo[], setHandler: React.Dispatch<React.SetStateAction<UrlInfo[]>>, newUrl: string) {
  setHandler([...handler, urlToInfo(newUrl)]);
}

export function discreetlyChangeUrlPath(path: string) {
  window.history.replaceState(null, "", path)
}

export function getAnimationState(urlsHandler: UrlInfo[], appName: string): AnimationStates {
  let inSecondPart = false;
  let inFirstPart = false;
  console.log("for ", urlsHandler)
  for (let i = 1; i < urlsHandler.length; i++) {
    if (urlsHandler[i].app === appName) {
      inSecondPart = true;
    }
  }
  if (urlsHandler[0] && urlsHandler[0].app === appName) {
    inFirstPart = true;
  }
  if (inFirstPart && inSecondPart) {
    return "intro";
  }
  if (inFirstPart && !inSecondPart && urlsHandler.length > 1) {
    return "outro";
  } else {
    console.log("no bcse", inFirstPart, inSecondPart, urlsHandler.length)
  }
  if (inFirstPart) {
    return "inter";
  }
  if (inSecondPart) {
    return "intro";
  }
  return "off";
}