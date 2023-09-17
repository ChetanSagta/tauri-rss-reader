import './Navbar.css';
import { invoke } from "@tauri-apps/api/tauri";
import { useState } from "react";

export default function Navbar(props) {

  const [urlText, setUrlText] = useState("");

  function search() {
  }

  function addURL() {
    console.log("Add URL function");

    invoke('add_new_rss_feed', { url: urlText })
      .then(message => { console.log("Success: ", message) })
      .catch(message => { console.log("Error: ", message) });

    setUrlText("");
  }

  return (
    <>
      <div id="navbar">
        <div id="addUrlBar">
          <input type="text" id="addUrlText" placeholder="Add URL" onChange={(event) => { setUrlText(event.target.value) }} />
          <input type="button" id="addURL" value="OK" onClick={addURL} />
        </div>
        <div id="searchBar">
          <input type="text" id="searchText" placeholder="Search" />
          <input type="button" value="Submit" onClick={search} />
        </div>
      </div>
    </>
  );
}
