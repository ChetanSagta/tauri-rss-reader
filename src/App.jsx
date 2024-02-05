import "./App.css";
import MainContent from "./components/MainContent";
import Navbar from "./components/Navbar";
import SideBar from "./components/SideBar";
import Content from "./components/Content";
import { useState } from "react";

export default function App() {


  const [siteUrl, selectSiteUrl] = useState('');
  const [detailUrl, selectDetailUrl] = useState('');

  return (
    <>
      <Navbar />
      <div id="body">
        <SideBar chooseSite={(url) => { selectSiteUrl(url); }} />
        <MainContent feedsUrl={siteUrl} chooseUrl={(url) => { selectDetailUrl(url); }} />
        <Content url={detailUrl} />
      </div>
    </>
  );
}
