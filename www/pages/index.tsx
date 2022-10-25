import type { NextPage } from 'next'
import init, { greet, foo } from "wasm-react";
import {useEffect, useState} from "react";


const Home: NextPage = () => {
  const [res, setRes] = useState('')
  useEffect(() => {
    init().then(() => {
      const g = greet("WebAssembly")
      console.log(`g ${g}`)
      setRes(`${g} ${foo(1, 100)}`)
    })
  }, [setRes])
  return (
    <div>
      {res}
    </div>
  )
}

export default Home
