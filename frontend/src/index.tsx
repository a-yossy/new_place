import { createRoot } from "react-dom/client";
import { Main } from "./main";
import "./index.css";
import "@mantine/core/styles.css";
import { MantineProvider } from "@mantine/core";

const container = document.querySelector("#root") as Element;
const root = createRoot(container);

root.render(
  <MantineProvider>
    <Main />
  </MantineProvider>
);
