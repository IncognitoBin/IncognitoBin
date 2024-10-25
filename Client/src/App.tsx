import "./App.css";
import { ThemeProvider } from "@/components/theme-provider";
import { BrowserRouter as Router, Route, Routes } from "react-router-dom";
import CreatePaste from "./pages/Create";
import ReadPaste from "./pages/Read";
import { Toaster } from "./components/ui/toaster";

function App() {
  
  return (
    <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
      <Router>
        <Routes>
          <Route path="/Create" element={<CreatePaste />} />
          <Route path="/:id" element={<ReadPaste />} />
          <Route path="/:id/:key" element={<ReadPaste />} />
          <Route path="/:id/:key/:iv" element={<ReadPaste />} />
        </Routes>
      </Router>
      <Toaster />
    </ThemeProvider>
  );
}

export default App;
