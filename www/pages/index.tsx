import type { NextPage } from 'next'
import init, { greet, add, arr_sum as arraySum } from 'wasm-react'
import { useEffect, useState } from 'react'

const Home: NextPage = () => {
  const [res, setRes] = useState(() => <></>)
  useEffect(() => {
    init().then(() => {
      const greeting = greet('WebAssembly')
      const x = add(99, 1)
      const y = arraySum()
      setRes(() => (
        <div
          style={{
            display: 'flex',
            flexDirection: 'column',
            justifyContent: 'center',
            alignItems: 'center',
            width: '100wv',
            height: '100vh',
          }}
        >
          <span>{greeting}</span>
          <span>99 + 1 = ${x}</span>
          <span>{y}</span>
        </div>
      ))
    })
  }, [setRes])
  return <div>{res}</div>
}

export default Home
