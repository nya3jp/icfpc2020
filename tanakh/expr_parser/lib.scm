(define org-cons cons)
(define org-car car)
(define org-cdr cdr)

(define (i) (lambda (x) x))

(define (t) (lambda (x0) (lambda (x1) x0)))
(define (f) (lambda (x0) (lambda (x1) x1)))

(define (add) (lambda (x) (lambda (y) (+ x y))))
(define (neg) (lambda (x) (- x)))
(define (mul) (lambda (x) (lambda (y) (* x y))))
(define (div) (lambda (x) (lambda (y) (/ x y))))

(define (eq) (lambda (x) (lambda (y) (if (= x y) (t) (f)))))
(define (lt) (lambda (x) (lambda (y) (if (< x y) (t) (f)))))

(define (s) (lambda (x) (lambda (y) (lambda (z) ((x z) (y z))))))
(define (c) (lambda (x) (lambda (y) (lambda (z) ((x z) y)))))
(define (b) (lambda (x) (lambda (y) (lambda (z) (x (y z))))))

(define (cons) (lambda (x) (lambda (y) (lambda (z) ((z x) y)))))
(define (nil) (lambda (x) (t)))
(define (isnil) (lambda (x)
    (x (lambda (x0) (lambda (x1) (f))))))

;;;

(define (car) (lambda (x) (x (t))))
(define (cdr) (lambda (x) (x (f))))
(define (vec) (cons))

(define (ifzero)
    (lambda (b) (lambda (x0) (lambda (x1)
    
(((((eq) b) 0) x0) x1)

))))

(define (send) (lambda (v)
    (display "Send: ")
    (display (to-scm-value v))
    (newline)

    (let ((t (process (format "cargo run -q -- send \"~a\" 2>> log.txt" (to-scm-value v)))))
        (define v (read (org-car t)))
        (display v)
        (newline)
        (from-scm-value v))))

(define (to-scm-value x)
    (if (integer? x) x
        (if ((((isnil) x) #t) #f) '()
            (org-cons (to-scm-value ((car) x))
                (to-scm-value ((cdr) x))))))

(define (from-scm-value x)
    (if (integer? x) x
        (if (null? x) (nil)
            (((cons)
                (from-scm-value (org-car x)))
                (from-scm-value (org-cdr x))))))
