pub mod config;
pub mod client;
// TODO:
// - [ ] Implement aws client which uses the api_key and makes a put request, i.e. PUT
// https:://api.amazon.com/v1/strikes/spoc/{name}, This will return a json response with the
// following structure:
// {
//   "name": "name",
//   "strikes": 1,
// }
// - [ ] If the request was successful, print the current strike count for the name on the console
// - [ ] If the request was not successful, print: "Failed to update strike count for {name}", as a
// fallback you can cache the strike count in a file and try to update it again in the next run
// - [ ] Implement "strike ls" command which will print the strike count for the team members
// - [ ] Implement "strike reset" command which will reset the strike count for the team members
//
// - [ ] Make the client available as a binary to download from github releases
// - [ ] Add a github action to build and release the binary on every push to the main branch
// - [ ] Add a github action to run the tests on every push to the main branch
// - [ ] Use brew to install the binary on mac, you need to create a tap for this, i.e. a new
// repository with the formula to install the binary using brew install <repo> command on mac

