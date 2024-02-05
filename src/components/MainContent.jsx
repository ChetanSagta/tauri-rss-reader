import { invoke } from '@tauri-apps/api/tauri'
import { useEffect, useState } from 'react';
import { Item } from '../models/item';
import Card from './Card.jsx';

import "./MainContent.css"

export default function MainContent(props) {

  const [items, setItems] = useState([]);

  if (props == null) {
    return;
  }
  
  var url = props.feedsUrl;

  useEffect(() => {
    invoke('get_feed_data', { url: url })
      .then(response => {
        var temp_items = []
        for (const res of response) {
          let category = res['category'].split('|');
          let creator = res['creator'];
          let description = res['description'];
          let link = res['link'];
          let pub_date = res['pub_date'];
          let title = res['title'];
          var item = new Item(category, -1, creator, description, link, pub_date, title);
          temp_items.push(item);
        setItems(temp_items);
        }
      }).catch(error => { console.log("Error: ", error); })
  }, [url]);

  return (
    <div id="mainBody">
      {items.map((item, index) => { return <Card item={item} key= {index} selectUrl={(detailUrl) => {props.chooseUrl(detailUrl);}}/> })}
    </div>
  );
}
