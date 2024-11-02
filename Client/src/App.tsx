import "./App.css";
import { ThemeProvider } from "@/components/theme-provider";
import { BrowserRouter as Router, Route, Routes } from "react-router-dom";
import CreatePaste from "./pages/Create";
import ReadPaste from "./pages/Read";
import { Toaster } from "./components/ui/toaster";
import AuthPage from "./pages/Auth";
import MyPastes from "./pages/MyPastes";
import Header from "./layouts/Header";

function App() {
  return (
    <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
      <Router>
      <Header />
        <Routes>
          <Route path="/" element={<CreatePaste />} />
          <Route path="/Create" element={<CreatePaste />} />
          <Route path="/:id" element={<ReadPaste />} />
          <Route path="/:id/:key" element={<ReadPaste />} />
          
          <Route path="/:id/:key/:iv" element={<ReadPaste />} />
          <Route path="/Auth" element={<AuthPage />} />
          <Route path="/Auth/:id" element={<AuthPage />} />
          <Route path="/Me" element={<MyPastes />} />
          
        </Routes>
      </Router>
      <Toaster />
    </ThemeProvider>
  );
}

export default App;
