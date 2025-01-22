fn main (){
	let traffic_light = "black";

	match traffic_light {
	"red" => println! ("Stop!"),
	"yellow" => println! ("Get Ready"),
	"green"  => println! ("Go!"),
	_ => println! ("Invalid Color"),
	}
}

