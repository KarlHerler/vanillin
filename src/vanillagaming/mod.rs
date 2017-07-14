// vanillagaming curler
// do curl http://db.vanillagaming.org/?search=[urlencodded thing]
// if response is 302, use link
// if 200 check if nothing found, return "nothing found, sorry"
// if 200 check if many things found, return that
use std::io::{self, Write};
use std::string::String;
use futures::{Future, Stream, IntoFuture};
use futures::future;
use hyper::{Client, Request, Method, Error};
use tokio_core::reactor::Core;


/// Public: Find searches db.vanillagaming.org for the input, if we get a 302 redirect
/// We know we've matched precisely with one thing and return the direct link in the "Location"
/// header (the destination of the redirect). If we did not get a 302 we see if the body
/// is a set of multiple items (search results) or a "No results found" and return appropriately
pub fn find(thing: &str) -> Option<String> {
  println!("Vanillagaming::find called with: {}", thing);
  let origin = "http://db.vanillagaming.org/";

  // TODO: Perhaps move this initialization into a module for perf
  let mut core = Core::new().unwrap();
  let client = Client::new(&core.handle());


  let uri_str = origin.to_owned()+"?search="+&search_encode(thing);
  let uri = uri_str.parse().unwrap();
  println!("Making request to: {}", &uri_str);
  let req = Request::new(Method::Get, uri);



  let work = client.request(req).and_then(|res| {
    println!("Response Code: {}", res.status());
    match res.headers().get_raw("Location") {
      None =>  {
        println!("Got no redirect");
      },
      Some(dest) => {
        println!("Got redirect");
        let d = String::from_utf8(dest.one().unwrap().to_vec()).unwrap(); // lovely
        return future::ok::<_,Error>(Some(String::from(origin)+&d)).boxed()
      }
    };

    // Ok we didn't get any precise match so we do some more investigation of what we have
    res.body().fold(Vec::new(), |mut v, chunk| {
      // translate the chunks into a vector of chunks
      v.extend(&chunk[..]);
      future::ok::<_, Error>(v)
    }).and_then(|chunks| {
      // translate the vector of chunks into a string
      let s = String::from_utf8(chunks).unwrap();
      if got_search_results_from_remote(&s) { return future::ok::<_, Error>(Some(uri_str)) }
      future::ok::<_, Error>(None)
    }).boxed()
  });

  return core.run(work).unwrap();
}

fn search_encode(thing: &str) -> String {
  thing.replace(" ","+")
}

/// Private: Vanillagaming doesn't bother with HTTP return codes so we need to do
/// some searching, can't be bothered with a proper HTML parser so we just do a quick string search.
fn got_search_results_from_remote(body: &String) -> bool {
  !body.contains("No results for ")
}
