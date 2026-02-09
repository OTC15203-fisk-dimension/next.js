'use client'

import type { JSX } from 'react'
import type { ErrorComponent } from './error-boundary'
import type { FallbackComponent } from './catch-error'
import { ErrorBoundary } from './error-boundary'

export function CatchErrorBoundary<P extends Record<string, any>>({
  fallback,
  componentProps,
  children,
}: {
  fallback: FallbackComponent<P>
  componentProps: P
  children?: React.ReactNode
}): JSX.Element {
  const errorComponent: ErrorComponent = (errorInfo) =>
    fallback(componentProps, errorInfo)

  return (
    <ErrorBoundary errorComponent={errorComponent}>{children}</ErrorBoundary>
  )
}
