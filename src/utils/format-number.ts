export function formatNumber(num: number) {
  return num.toLocaleString('ru', {
    minimumFractionDigits: 1,
    maximumFractionDigits: 1,
  })
}
