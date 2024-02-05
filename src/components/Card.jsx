import "./Card.css"

export default function Card(props) {


  let prop = props.item;
  let category = prop['category'];

  const category_array = [];
  let count = 0;
  category.forEach(c => {
    if (c == "") return;
    category_array.push(
      <span className="category" key={++count}
        onClick={(event) => {
          console.log("Category Selected: ", event.currentTarget.innerHTML);
        }}>{c}</span>);
  })

  return (
    <div id="card">
      <div className="title" href={prop.link} onClick={() => {
        console.log(prop.link);
        props.selectUrl(prop.link);
      }}>{prop.title}({prop.pub_date})</div>
      {category_array}
      <div className="description">{prop.description}</div>
    </div>
  )
}
