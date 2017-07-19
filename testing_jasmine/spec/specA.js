describe("Array", function() {
    var arr;
    it("can be created using `new`", function () {
        arr = new Array();
        expect(arr instanceof Array).toBe(true)
        expect(arr.length).toBe(0);
    });
    describe("push", function() {
        it("adds an element", function () {
            arr.push("element one");
            expect(arr.length).toBe(1);
            expect(arr[0]).toBe("element one");
        });
        it("inserts new elements at the end (highest index)", function () {
            arr.push("element two");
            expect(arr.length).toBe(2);
            expect(arr[0]).toBe("element one");
            expect(arr[1]).toBe("element two");
        });
    });
});

