import { catchError } from 'next/navigation'
import { ErrorFallback } from './catch-error-wrapper'

// catchError can be called from RSC
const ErrorWrapper = catchError(ErrorFallback)

export default function Layout({ children }: { children: React.ReactNode }) {
  // A prop can be passed from the RSC to the error component
  const title = 'server-catch-error'
  return <ErrorWrapper title={title}>{children}</ErrorWrapper>
}
