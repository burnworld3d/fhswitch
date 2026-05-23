import i18n from "i18next";
import { initReactI18next } from "react-i18next";

import en from "./locales/en.json";
import ja from "./locales/ja.json";
import zh from "./locales/zh.json";

type Language = "zh" | "en" | "ja";

const DEFAULT_LANGUAGE: Language = "en";

const getInitialLanguage = (): Language => {
  if (typeof window !== "undefined") {
    try {
      const stored = window.localStorage.getItem("language");
      if (stored === "zh" || stored === "en" || stored === "ja") {
        return stored;
      }
    } catch (error) {
      console.warn("[i18n] Failed to read stored language preference", error);
    }
  }

  return DEFAULT_LANGUAGE;
};

const resources = {
  en: {
    translation: en,
  },
  ja: {
    translation: ja,
  },
  zh: {
    translation: zh,
  },
};

i18n.use(initReactI18next).init({
  resources,
  lng: getInitialLanguage(),
  fallbackLng: "en",

  interpolation: {
    escapeValue: false, // React 已经默认转义
  },

  // 开发模式下显示调试信息
  debug: false,
});

export default i18n;
