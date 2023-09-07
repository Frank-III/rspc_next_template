import '@/styles/globals.css'
import type { AppProps } from 'next/app'
import { ReactQueryDevtools } from '@tanstack/react-query-devtools';
import React from "react";
import { rspc, client, queryClient } from "../utils/rspc";

export default function App({ Component, pageProps }: AppProps) {
  return (
    <rspc.Provider client={client} queryClient={queryClient}>
      <>
        <Component {...pageProps} />
        <ReactQueryDevtools initialIsOpen={false} />
      </>
    </rspc.Provider>)
}
