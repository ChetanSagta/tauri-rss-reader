import "./App.css";
import Navbar from "./components/Navbar";
import SideBar from "./components/SideBar";

export default function App() {

  return (
    <>
      <Navbar />
      <div id="body">
      <SideBar />
      </div>
    </>
  );
}
