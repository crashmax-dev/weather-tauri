import Weather01d from '~icons/weather/01d'
import Weather01n from '~icons/weather/01n'
import Weather02d from '~icons/weather/02d'
import Weather02n from '~icons/weather/02n'
import Weather03 from '~icons/weather/03'
import Weather04 from '~icons/weather/04'
import Weather09 from '~icons/weather/09'
import Weather10d from '~icons/weather/10d'
import Weather10n from '~icons/weather/10n'
import Weather11 from '~icons/weather/11'
import Weather13 from '~icons/weather/13'
import Weather50 from '~icons/weather/50'
import type { FunctionalComponent, SVGAttributes } from 'vue'

type Icon = FunctionalComponent<SVGAttributes>

export const WEATHER_ICONS: Record<string, Icon> = {
  '01d': Weather01d,
  '01n': Weather01n,
  '02d': Weather02d,
  '02n': Weather02n,
  '03d': Weather03,
  '03n': Weather03,
  '04d': Weather04,
  '04n': Weather04,
  '09d': Weather09,
  '09n': Weather09,
  '10d': Weather10d,
  '10n': Weather10n,
  '11d': Weather11,
  '11n': Weather11,
  '13d': Weather13,
  '13n': Weather13,
  '50d': Weather50,
  '50n': Weather50,
}
