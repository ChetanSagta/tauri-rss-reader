import { invoke } from '@tauri-apps/api/tauri'

import "./SideBar.css";
import { useState, useEffect } from "react";
import MainContent from "./MainContent";
import { Button } from 'react-bootstrap';

export default function SideBar() {

  const [links, setLinks] = useState([]);
  const [selectedLink, setSelectedLink] = useState('');

  useEffect(() => {
    console.log("Use Effect Called");
    invoke('get_all_feed_names')
      .then(response => { setLinks(response); })
      .catch(error => { console.log("Error: ", error); })
  }, []);

  function refreshFeed(link) {
    invoke('refresh_feed', { name: link })
      .then()
      .catch(error => { console.log("Error: ", error); })
  }

  function refreshAllFeeds() {
    invoke('get_all_feed_names_from_file')
      .then()
      .catch(error => { console.log("Error: ", error); })
  }


  const link_array = [];
  let count = 0;
  links.forEach(link => {
    if (link == "") return;
    link_array.push(
      <div key={++count} className="link">
        <div onClick={() => {
          setSelectedLink(link.link);
        }}>{link.title}</div>
        <Button type="button" variant="primary" onClick={()=>refreshFeed(link.link)} className="refreshBtn">
          REFRESH
        </Button>
      </div>);
  })

  return (
    <>
      <div id="sidebar">
        {link_array}
        <Button type="button" variant="primary" onClick={refreshAllFeeds} className="refreshBtn">
          REFRESH ALL
        </Button>
      </div>
      <MainContent link={selectedLink} />
    </>
  );
}
