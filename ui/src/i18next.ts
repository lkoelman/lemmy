import i18n from 'i18next';
import { initReactI18next } from "react-i18next";

import { en } from './translations/en';
import { el } from './translations/el';
import { eu } from './translations/eu';
import { eo } from './translations/eo';
import { es } from './translations/es';
import { de } from './translations/de';
import { fr } from './translations/fr';
import { sv } from './translations/sv';
import { ru } from './translations/ru';
import { zh } from './translations/zh';
import { nl } from './translations/nl';
import { it } from './translations/it';
import { fi } from './translations/fi';
import { ca } from './translations/ca';
import { fa } from './translations/fa';
import { hi } from './translations/hi';
import { pt_BR } from './translations/pt_BR';
import { ja } from './translations/ja';
import { ka } from './translations/ka';

// https://github.com/nimbusec-oss/inferno-i18next/blob/master/tests/T.test.js#L66
const resources = {
  en,
  el,
  eu,
  eo,
  es,
  ka,
  hi,
  de,
  zh,
  fr,
  sv,
  ru,
  nl,
  it,
  fi,
  ca,
  fa,
  pt_BR,
  ja,
};

function format(value: any, format: any, lng: any): any {
  return format === 'uppercase' ? value.toUpperCase() : value;
}

i18n
  .use(initReactI18next) // passes i18n down to react-i18next
  .init({
    debug: false,
    // load: 'languageOnly',

    // initImmediate: false,
    lng: 'en', // TODO: implement getLanguage(), see lemmy
    fallbackLng: 'en',
    resources,
    interpolation: { format },
  });

export { i18n, resources };
