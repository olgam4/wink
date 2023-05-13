interface Props {
  request: Request
}

export async function POST({ request }: Props) {
  const body = await request.json()

  console.log(body)

  const response = await fetch('http://127.0.0.1:8000/wink', {
    method: 'POST',
    body: JSON.stringify({ url: body.url }),
  })

  const data = await response.text()

  return new Response(JSON.stringify({ id: data }), {
    headers: {
      'content-type': 'application/json;charset=UTF-8',
    },
  })
}

export async function GET({ request }: Props) {
  const url = new URL(request.url)
  const id = url.searchParams.get('id')

  const response = await fetch('http://127.0.0.1:8000/wink/' + id)
  const data = await response.text()

  return new Response(data)
}

