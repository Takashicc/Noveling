import { MantineProvider } from "@mantine/core";
import { AppProps } from "next/app";
import Head from "next/head";

export default function App({ Component, pageProps }: AppProps) {
  return (
    <>
      <Head>
        <title>Noveling</title>
        <meta name='viewport' content='initial-scale=1, width=device-width' />
      </Head>

      <MantineProvider
        withCSSVariables={true}
        withGlobalStyles={true}
        withNormalizeCSS={true}
        theme={{ fontFamily: "Zen Kaku Gothic New", colorScheme: "light" }}
      >
        <Component {...pageProps} />
      </MantineProvider>
    </>
  );
}
