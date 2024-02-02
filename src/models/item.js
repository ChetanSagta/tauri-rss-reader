export class Item {
  
  getDate(milliseconds){

    let date = new Date(milliseconds);
    let hour = date.getHours();
    let minutes = date.getMinutes();

    return hour+":"+minutes;

  }

  constructor(category, channel_id, creator, description, link, pub_date,title) {
    this.category = category;
    this.creator = creator;
    this.description = description;
    this.link = link;
    this.pub_date = this.getDate(pub_date);
    this.title = title;
    this.channel_id = channel_id;
  }
}
