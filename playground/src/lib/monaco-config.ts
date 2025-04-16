import loader from "@monaco-editor/loader";

// Configure Monaco to use local files instead of CDN
loader.config({
  paths: {
    vs: "/nu-on-web/assets/vs",
  },
});

