import './App.css'
import { ThemeProvider } from "@/components/theme-provider"
import { BrowserRouter as Router, Route, Routes } from 'react-router-dom';
import CreatePaste from './pages/Create';

function App() {

  return (
    <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
     <Router>
      <Routes>
        <Route path="/Create" element={<CreatePaste />} />
      </Routes>
    </Router>
    
    </ThemeProvider>
  );
}

export default App
