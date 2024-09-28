export function load({ data, url }) {
    let pathname = url.pathname;
    return {
        ...data,
        pathname
    }
}