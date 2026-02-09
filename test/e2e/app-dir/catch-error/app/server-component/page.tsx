import { Suspense } from 'react'
import { connection } from 'next/server'

export default function Page() {
  return (
    <Suspense>
      <PageImpl />
    </Suspense>
  )
}

async function PageImpl() {
  await connection()
  throw new Error('this is a test')

  return null
}
