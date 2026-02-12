import { createContext } from 'react'
import type { ValidationBoundaryTracking } from './boundary-tracking'

// NOTE: this is externalized so that `app-render` and `./boundary.tsx` get the same instant.
// `./boundary.tsx` is "use client" and referenced from the server `./instant-validation.tsx`,
// and that seems to cause module duplication if this this isn't pulled out into a singleton.
export const InstantValidationBoundaryTrackingContext =
  createContext<ValidationBoundaryTracking | null>(null)
