import {get_descriptor_types, get_script_type, get_threshold_count} from "miniscript-shim";

const UNICODE_LINE = /\r\n|(?!\r\n)[\n-\r\x85\u2028\u2029]/;

const descriptor = "sh(wsh(sortedmulti(1,xpub661MyMwAqRbcFW31YEwpkMuc5THy2PSt5bDMsktWQcFF8syAmRUapSCGu8ED9W6oDMSgv6Zz8idoc4a6mr8BDzTJY47LJhkJ8UB7WEGuduB/1/0/*,xpub69H7F5d8KSRgmmdJg2KhpAK8SR3DjMwAdkxj3ZuxV27CprR9LgpeyGmXUbC6wb7ERfvrnKZjXoUmmDznezpbZb7ap6r1D3tgFxHmwMkQTPH/0/0/*)))"

const result = descriptor.split(UNICODE_LINE)
            .flatMap((v)=> {
                if (v.match(UNICODE_LINE)) return [];
                try {
                    /// TODO: Cache based on V
                    const s = get_script_type(v);
                    return [['ok', v, s]];
                } catch (e) {
                    if (typeof e === 'string') return [['err', v, e]];
                    else throw e;
                }
            });
console.log(get_descriptor_types())
console.log(result)
console.log(`threshold: ${get_threshold_count(descriptor)}`)