import { Provider } from '@locales'

import { createInput } from '@primitives/form/input'
import Paragraph from '@components/paragraph'
import Form from '@primitives/form'

export default function() {
  const urlInput = createInput('url')
  const [newId, setNewId] = createSignal('')

  return (
    <Provider>
      <Title>wink</Title>
      <div class="full flex-center flex-col bg-gray-100/75 dark:bg-gray-800 transition-colors">
        <Paragraph class="-mt-10 text-gray-500" key="batman" />
        <div class="flex mt-6 space-x-2 items-center">
          <Form
            inputs={[urlInput]}
            onSubmit={async () => {
              const wink = await fetch('/api/url', {
                method: 'POST',
                body: JSON.stringify({ url: urlInput.value() }),
              })

              const { id } = await wink.json()

              setNewId(id)
            }}
          />
        </div>
        <div>
          {newId()}
        </div>
      </div>
    </Provider>
  )
}
