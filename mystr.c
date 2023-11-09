#include <limits.h>

void *memcpy(void *dst, const void *src, unsigned long count);

int memcmp(const void *vl, const void *vr, unsigned long n) {
    const unsigned char *l = vl, *r = vr;
    for (; n && *l == *r; n--, l++, r++)
        ;
    return n ? *l - *r : 0;
}

void *memchr(const void *src, int c, unsigned long n) {
    const unsigned char *s = src;
    c = (unsigned char)c;
    for (; n && *s != c; s++, n--)
        ;
    return n ? (void *)s : 0;
}

void *memmove(void *dest, const void *src, unsigned long n) {
    char *d = dest;
    const char *s = src;

    if (d == s)
        return d;
    if ((unsigned long)s - (unsigned long)d - n <= -2 * n)
        return memcpy(d, s, n);

    if (d < s) {
        for (; n; n--)
            *d++ = *s++;
    } else {
        while (n)
            n--, d[n] = s[n];
    }

    return dest;
}

unsigned long strlen(const char *s) {
    const char *a = s;
    for (; *s; s++)
        ;
    return s - a;
}

unsigned long strnlen(const char *s, unsigned long n) {
    const char *p = memchr(s, 0, n);
    return p ? (unsigned long)(p - s) : n;
}

static char *strchrnul(const char *s, int c) {
    c = (unsigned char)c;
    if (!c)
        return (char *)s + strlen(s);
    for (; *s && *(unsigned char *)s != c; s++)
        ;
    return (char *)s;
}

char *strchr(const char *s, int c) {
    char *r = strchrnul(s, c);
    return *(unsigned char *)r == (unsigned char)c ? r : 0;
}

static void *memrchr(const void *m, int c, unsigned long n) {
    const unsigned char *s = m;
    c = (unsigned char)c;
    while (n--)
        if (s[n] == c)
            return (void *)(s + n);
    return 0;
}

// From https://git.musl-libc.org/cgit/musl/tree/src/ctype

char *strrchr(const char *s, int c) {
    return memrchr(s, c, strlen(s) + 1);
}

int isdigit(int c) {
    return (unsigned)c - '0' < 10;
}

int isalpha(int c) {
    return ((unsigned)c | 32) - 'a' < 26;
}

int isupper(int c) {
    return (unsigned)c - 'A' < 26;
}

int isspace(int c) {
    return c == ' ' || (unsigned)c - '\t' < 5;
}

// From https://github.com/openbsd/src/blob/master/lib/libc/stdlib/strtoul.c

unsigned long strtoul(const char *nptr, char **endptr, int base) {
    const char *s;
    unsigned long acc, cutoff;
    int c;
    int neg, any, cutlim;
    s = nptr;
    do {
        c = (unsigned char)*s++;
    } while (isspace(c));
    if (c == '-') {
        neg = 1;
        c = *s++;
    } else {
        neg = 0;
        if (c == '+')
            c = *s++;
    }
    if ((base == 0 || base == 16) && c == '0' && (*s == 'x' || *s == 'X')) {
        c = s[1];
        s += 2;
        base = 16;
    }
    if (base == 0)
        base = c == '0' ? 8 : 10;
    cutoff = ULONG_MAX / (unsigned long)base;
    cutlim = ULONG_MAX % (unsigned long)base;
    for (acc = 0, any = 0;; c = (unsigned char)*s++) {
        if (isdigit(c))
            c -= '0';
        else if (isalpha(c))
            c -= isupper(c) ? 'A' - 10 : 'a' - 10;
        else
            break;
        if (c >= base)
            break;
        if (any < 0)
            continue;
        if (acc > cutoff || (acc == cutoff && c > cutlim)) {
            any = -1;
            acc = ULONG_MAX;
            // errno = ERANGE;
        } else {
            any = 1;
            acc *= (unsigned long)base;
            acc += c;
        }
    }
    if (neg && any > 0)
        acc = -acc;
    if (endptr != 0)
        *endptr = (char *)(any ? s - 1 : nptr);
    return (acc);
}
