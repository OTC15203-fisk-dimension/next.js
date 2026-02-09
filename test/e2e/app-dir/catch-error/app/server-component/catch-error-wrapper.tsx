'use client'
import type { ErrorInfo } from 'next/navigation'

export function ErrorFallback(
  props: { title: string },
  { error, componentStack, ownerStack, reset, retry }: ErrorInfo
) {
  return (
    <>
      <p id="error-boundary-message">{error.message}</p>
      <p id="error-boundary-title">{props.title}</p>
      <p id="error-component-stack">{componentStack}</p>
      <p id="error-owner-stack">{ownerStack}</p>
      <button id="reset" onClick={() => reset()}>
        Reset
      </button>
      <button id="retry" onClick={() => retry()}>
        Retry
      </button>
    </>
  )
}
