import { RouterProvider, createRouter } from "@tanstack/react-router";
import { StrictMode } from "react";
import ReactDOM from "react-dom/client";
import "./app.css";
import "@mantine/core/styles.css";
import { MantineProvider } from "@mantine/core";
import { Client, Provider, cacheExchange, fetchExchange } from "urql";

import { routeTree } from "./routeTree.gen";

const router = createRouter({ routeTree });
const client = new Client({
  url: "http://localhost:8000/graphql",
  exchanges: [cacheExchange, fetchExchange],
});

declare module "@tanstack/react-router" {
  interface Register {
    router: typeof router;
  }
}

const rootElement = document.getElementById("root");
if (rootElement !== null && !rootElement.innerHTML) {
  const root = ReactDOM.createRoot(rootElement);
  root.render(
    <StrictMode>
      <Provider value={client}>
        <MantineProvider>
          <RouterProvider router={router} />
        </MantineProvider>
      </Provider>
    </StrictMode>
  );
}
