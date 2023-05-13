import Paragraph from '@components/paragraph'
import { Provider } from '@locales'
import { Link } from '@solidjs/router'

export default function() {
  createEffect(async () => {
    const id = window.location.href.split('/').pop()
    const response = await fetch('/api/url?id=' + id)

    const newLocation = await response.text()
    newLocation && window.location.replace('http://' + newLocation)
  })

  return (
    <Provider>
      <Title>404</Title>
      <div class="full flex-center flex-col bg-gray-100/75 text-gray-400 dark:bg-gray-800">
        <h1 class="text-4xl font-extrabold">404</h1>
        <div class="flex space-x-1 mt-1">
          <Paragraph key='404' />
          <div class="pt-1">
            <div class="i-carbon-face-dissatisfied-filled h-4 w-4" />
          </div>
        </div>
        <Link href="/" class="flex items-center space-x-1 mt-4 transition hover:(text-gray-400/40)">
          <div class="i-carbon-arrow-left h-7 w-7" />
          <span class="font-extrabold">
            <Paragraph key='goback' />
          </span>
        </Link>
      </div>
    </Provider>
  )
}
