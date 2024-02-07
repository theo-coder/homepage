async function main() {
    const wallpaper_res = await fetch("/wallpaper");
    const wallpaper_url = await wallpaper_res.json();

    document.querySelector(".wallpaper").style.backgroundImage = `url(${wallpaper_url})`;

    const form = document.getElementById("searchForm");

    form.addEventListener("submit", (e) => {
        e.preventDefault();

        const formData = new FormData(form);
        const queryString = formData.get("q");

        window.history.pushState("", "", `?q=${queryString}`);
        searchResult();
    });


    searchResult();

}

function searchResult() {
    const urlParams = new URLSearchParams(window.location.search);
    const queryString = urlParams.get("q");

    form.querySelector("input").value = queryString;

    console.log(queryString);
}

await main()
