#include <objc/objc.h>
#include <objc/NSObject.h>

void RustObjCExceptionThrow(id exception) {
    @throw exception;
}

// We return `unsigned char`, since it is guaranteed to be an `u8` on all platforms
unsigned char RustObjCExceptionTryCatch(void (*try)(void *), void *context, id *error) {
    @try {
        try(context);
        if (error) {
            *error = nil;
        }
        return 0;
    } @catch (id exception) {
        if (error) {
            *error = [exception retain];
        }
        return 1;
    }
}
