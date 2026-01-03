import { Route, Switch } from "wouter";
import { ThemeProvider } from "@/contexts/ThemeContext";
import { AuthProvider } from "@/contexts/AuthContext";
import { Toaster } from "@/components/ui/sonner";
import Home from "@/pages/Home";
import TauriAutoUpdater from "@/components/TauriAutoUpdater";

export default function App( ) {
  return (
    <ThemeProvider defaultTheme="light" storageKey="fixmate-theme">
      <AuthProvider>
        <TauriAutoUpdater />
        <Switch>
          <Route path="/" component={Home} />
          <Route>
            <div className="min-h-screen flex items-center justify-center">
              <div className="text-center">
                <h1 className="text-4xl font-bold mb-4">404</h1>
                <p className="text-muted-foreground">Page not found</p>
              </div>
            </div>
          </Route>
        </Switch>
        <Toaster />
      </AuthProvider>
    </ThemeProvider>
  );
}
