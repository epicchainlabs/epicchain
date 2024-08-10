import i18n from "i18next";
import { initReactI18next } from "react-i18next";
import HttpApi from "i18next-http-backend";
import Config from "./config";

const language = Config.Language || navigator.language.split(/[-_]/)[0]; // language without region code
const FALLBACKLNG = "en";

i18n
  // .use(Backend)
  // .use(initReactI18next) // passes i18n down to react-i18next
  .use(HttpApi)
  .use(initReactI18next)
  .init(
    {
      react: {
        useSuspense: true,
        // wait: true,
      },
      // resources,
      lng: language,
      // whitelist: [],
      fallbackLng: FALLBACKLNG,
      // ns: ['translation'],
      backend: {
        /* translation file path */
        loadPath: "locales/{{lng}}/{{ns}}.json",
      },
      // keySeparator: false, // we do not use keys in form messages.welcome
      interpolation: {
        escapeValue: false, // react already safes from xss
      },
    },
    (e, t) => {
      if (e) {
        console.log("loading language fail:", e);
        i18n.language = FALLBACKLNG;
      }
    }
  );

export default i18n;
